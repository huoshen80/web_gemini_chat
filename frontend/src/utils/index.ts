// API 配置
const getApiConfig = () => {
	// 判断是否为开发环境
	const isDev = import.meta.env.DEV;
	
	// 在开发环境使用固定地址，生产环境使用当前域名
	const baseUrl = isDev ? "http://127.0.0.1:23333" : window.location.origin;
	const wsProtocol = window.location.protocol === "https:" ? "wss:" : "ws:";
	const wsUrl = isDev 
		? "ws://127.0.0.1:23333/ws" 
		: `${wsProtocol}//${window.location.host}/ws`;

	return {
		WS_URL: wsUrl,
		API_URL: `${baseUrl}/api`,
		HEALTH_URL: `${baseUrl}/api/health`,
		UPLOAD_URL: `${baseUrl}/api/upload`,
	};
};

export const API_CONFIG = getApiConfig();

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
