import type { Message } from "../types";

const STORAGE_KEYS = {
	MESSAGES: "web_chat_messages",
	USER_ID: "web_chat_user_id",
} as const;

// 生成 UUID v4
function generateUUID(): string {
	return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, (c) => {
		const r = (Math.random() * 16) | 0;
		const v = c === "x" ? r : (r & 0x3) | 0x8;
		return v.toString(16);
	});
}

// 获取或创建用户 ID
export function getUserId(): string {
	try {
		let userId = localStorage.getItem(STORAGE_KEYS.USER_ID);
		if (!userId) {
			userId = generateUUID();
			localStorage.setItem(STORAGE_KEYS.USER_ID, userId);
		}
		return userId;
	} catch (e) {
		console.error("获取用户ID失败:", e);
		return generateUUID();
	}
}

// 保存消息到 localStorage
export function saveMessages(messages: Message[]): void {
	try {
		localStorage.setItem(STORAGE_KEYS.MESSAGES, JSON.stringify(messages));
	} catch (e) {
		console.error("保存消息失败:", e);
	}
}

// 从 localStorage 加载消息
export function loadMessages(): Message[] {
	try {
		const stored = localStorage.getItem(STORAGE_KEYS.MESSAGES);
		if (stored) {
			return JSON.parse(stored);
		}
	} catch (e) {
		console.error("加载消息失败:", e);
	}
	return [];
}

// 清除消息
export function clearMessages(): void {
	try {
		localStorage.removeItem(STORAGE_KEYS.MESSAGES);
	} catch (e) {
		console.error("清除消息失败:", e);
	}
}
