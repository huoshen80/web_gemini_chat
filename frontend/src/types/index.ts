// 消息类型
export interface Message {
	id: string;
	type: "user" | "model" | "system" | "thinking";
	content: string;
	model?: string;
	timestamp: number;
}

// 文件上下文
export interface FileContext {
	name: string;
	content: string;
	size: number;
}

// 连接状态
export type ConnectionStatus =
	| "connecting"
	| "connected"
	| "disconnected"
	| "error";

// 模型类型
export type ModelType = "flash-lite" | "flash-2.5" | "flash-3";

export interface ModelInfo {
	id: ModelType;
	name: string;
	description: string;
}

export const MODELS: ModelInfo[] = [
	{
		id: "flash-lite",
		name: "Gemini 2.5 Flash Lite",
		description: "轻量级模型，经济实惠，适合简单问答",
	},
	{
		id: "flash-2.5",
		name: "Gemini 2.5 Flash",
		description: "增强版 Flash，处理更长的上下文和多模态输入",
	},
	{
		id: "flash-3",
		name: "Gemini 3.0 Flash",
		description: "最新一代模型，速度更快、性能更强，适合各类对话场景",
	},
];

// WebSocket 消息类型
export interface WsChatMessage {
	type: "chat";
	data: { content: string };
}

export interface WsSetContextMessage {
	type: "set_context";
	data: { files: { name: string; content: string }[] };
}

export interface WsSwitchModelMessage {
	type: "switch_model";
	data: { model: string };
}

export interface WsClearContextMessage {
	type: "clear_context";
}

export type WsOutgoingMessage =
	| WsChatMessage
	| WsSetContextMessage
	| WsSwitchModelMessage
	| WsClearContextMessage;

// 服务器响应消息
export interface ServerResponseMessage {
	type: "response";
	data: { content: string; model: string };
}

export interface ServerThinkingMessage {
	type: "thinking";
	data: { content: string };
}

export interface ServerSystemMessage {
	type: "system";
	data: { content: string };
}

export interface ServerErrorMessage {
	type: "error";
	data: { content: string };
}

export interface ServerLoadingMessage {
	type: "loading";
	data: { is_loading: boolean };
}

export type ServerMessage =
	| ServerResponseMessage
	| ServerThinkingMessage
	| ServerSystemMessage
	| ServerErrorMessage
	| ServerLoadingMessage;
