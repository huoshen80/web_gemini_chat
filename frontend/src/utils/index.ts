// API 配置
export const API_CONFIG = {
	WS_URL: "ws://127.0.0.1:23333/ws",
	API_URL: "http://127.0.0.1:23333/api",
	HEALTH_URL: "http://127.0.0.1:23333/api/health",
	UPLOAD_URL: "http://127.0.0.1:23333/api/upload",
};

// 生成唯一 ID
export function generateId(): string {
	return `${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
}

// 格式化文件大小
export function formatFileSize(bytes: number): string {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

// 检查后端健康状态
export async function checkBackendHealth(): Promise<boolean> {
	try {
		const response = await fetch(API_CONFIG.HEALTH_URL);
		return response.ok;
	} catch {
		return false;
	}
}

// 上传文件
export async function uploadFiles(files: FileList): Promise<{
	success: boolean;
	files?: { name: string; content: string; size: number }[];
	error?: string;
}> {
	const formData = new FormData();
	for (let i = 0; i < files.length; i++) {
		formData.append("file", files[i]);
	}

	try {
		const response = await fetch(API_CONFIG.UPLOAD_URL, {
			method: "POST",
			body: formData,
		});
		const result = await response.json();

		if (!response.ok) {
			return { success: false, error: result.error || "上传失败" };
		}

		return { success: true, files: result.files };
	} catch (err) {
		return {
			success: false,
			error: err instanceof Error ? err.message : "上传失败",
		};
	}
}

// 导出存储相关函数
export {
	clearMessages,
	getUserId,
	loadMessages,
	saveMessages,
} from "./storage";
