import { useCallback, useEffect, useRef, useState } from "react";
import type {
	ConnectionStatus,
	FileContext,
	Message,
	ModelType,
	ServerMessage,
	WsOutgoingMessage,
} from "../types";
import {
	API_CONFIG,
	checkBackendHealth,
	generateId,
	getUserId,
	loadMessages,
	saveMessages,
} from "../utils";

export function useWebSocket() {
	const [socket, setSocket] = useState<WebSocket | null>(null);
	const [connectionStatus, setConnectionStatus] =
		useState<ConnectionStatus>("connecting");
	const [messages, setMessages] = useState<Message[]>(() => loadMessages());
	const [isLoading, setIsLoading] = useState(false);
	const [currentModel, setCurrentModel] = useState<ModelType>("flash");
	const [fileContexts, setFileContexts] = useState<FileContext[]>([]);
	const [error, setError] = useState<string>("");

	const isConnectingRef = useRef(false);
	const reconnectTimeoutRef = useRef<number | undefined>(undefined);
	const socketRef = useRef<WebSocket | null>(null);
	const userIdRef = useRef<string>(getUserId());

	// 发送消息到 WebSocket
	const sendMessage = useCallback((message: WsOutgoingMessage) => {
		if (socketRef.current?.readyState === WebSocket.OPEN) {
			// 添加用户 ID 到消息
			const messageWithUserId = { ...message, user_id: userIdRef.current };
			socketRef.current.send(JSON.stringify(messageWithUserId));
		}
	}, []);

	// 添加消息
	const addMessage = useCallback(
		(message: Omit<Message, "id" | "timestamp">) => {
			setMessages((prev) => {
				const newMessages = [
					...prev,
					{ ...message, id: generateId(), timestamp: Date.now() },
				];
				saveMessages(newMessages);
				return newMessages;
			});
		},
		[],
	);

	// 清除聊天记录
	const clearChat = useCallback(() => {
		setMessages([]);
		saveMessages([]);
	}, []);

	// 连接 WebSocket
	const connectWebSocket = useCallback(() => {
		if (isConnectingRef.current) return;

		isConnectingRef.current = true;
		setConnectionStatus("connecting");
		setError("");

		const ws = new WebSocket(API_CONFIG.WS_URL);
		socketRef.current = ws;

		ws.onopen = () => {
			console.log("WebSocket 已连接");
			isConnectingRef.current = false;
			setSocket(ws);
			setConnectionStatus("connected");
		};

		ws.onmessage = (event) => {
			try {
				const serverMsg: ServerMessage = JSON.parse(event.data);

				switch (serverMsg.type) {
					case "response":
						addMessage({
							type: "model",
							content: serverMsg.data.content,
							model: serverMsg.data.model,
						});
						break;
					case "thinking":
						addMessage({
							type: "thinking",
							content: serverMsg.data.content,
						});
						break;
					case "system":
						addMessage({
							type: "system",
							content: serverMsg.data.content,
						});
						break;
					case "error":
						setError(serverMsg.data.content);
						break;
					case "loading":
						setIsLoading(serverMsg.data.is_loading);
						break;
				}
			} catch {
				// 兼容旧格式
				addMessage({ type: "model", content: event.data });
			}
		};

		ws.onerror = (event) => {
			console.error("WebSocket 错误:", event);
			isConnectingRef.current = false;
			setConnectionStatus("error");
			setError("WebSocket 连接失败，请确保后端服务器正在运行");
		};

		ws.onclose = () => {
			console.log("WebSocket 已断开");
			isConnectingRef.current = false;
			setSocket(null);
			socketRef.current = null;
			setConnectionStatus("disconnected");

			if (reconnectTimeoutRef.current) {
				clearTimeout(reconnectTimeoutRef.current);
			}

			reconnectTimeoutRef.current = window.setTimeout(() => {
				checkBackendHealth().then((isHealthy) => {
					if (isHealthy) {
						connectWebSocket();
					} else {
						setError("后端服务器未响应");
						setConnectionStatus("error");
					}
				});
			}, 3000);
		};
	}, [addMessage]);

	// 发送聊天消息
	const sendChat = useCallback(
		(content: string) => {
			if (!content.trim()) return;

			addMessage({ type: "user", content });
			sendMessage({ type: "chat", data: { content } });

			// 发送消息后清除文件上下文
			if (fileContexts.length > 0) {
				setFileContexts([]);
			}
		},
		[addMessage, sendMessage, fileContexts.length],
	);

	// 设置文件上下文
	const setContextFiles = useCallback(
		(files: FileContext[]) => {
			setFileContexts(files);
			if (files.length > 0) {
				sendMessage({
					type: "set_context",
					data: {
						files: files.map((f) => ({ name: f.name, content: f.content })),
					},
				});
			}
		},
		[sendMessage],
	);

	// 清除文件上下文
	const clearContextFiles = useCallback(() => {
		setFileContexts([]);
		sendMessage({ type: "clear_context" });
	}, [sendMessage]);

	// 切换模型
	const switchModel = useCallback(
		(model: ModelType) => {
			setCurrentModel(model);
			sendMessage({ type: "switch_model", data: { model } });
		},
		[sendMessage],
	);

	// 清除错误
	const clearError = useCallback(() => {
		setError("");
	}, []);

	// 初始化连接
	useEffect(() => {
		checkBackendHealth().then((isHealthy) => {
			if (isHealthy) {
				connectWebSocket();
			} else {
				setConnectionStatus("error");
				setError("无法连接到后端服务器，请确保后端正在运行");
			}
		});

		return () => {
			if (reconnectTimeoutRef.current) {
				clearTimeout(reconnectTimeoutRef.current);
			}
			socketRef.current?.close();
		};
	}, [connectWebSocket]);

	return {
		socket,
		connectionStatus,
		messages,
		isLoading,
		currentModel,
		fileContexts,
		error,
		sendChat,
		setContextFiles,
		clearContextFiles,
		switchModel,
		clearError,
		clearChat,
		isConnected: connectionStatus === "connected",
	};
}
