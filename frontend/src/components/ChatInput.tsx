import { FileUp, Loader2, Send } from "lucide-react";
import type { FormEvent } from "react";
import { useRef, useState } from "react";
import type { FileContext } from "../types";
import { uploadFiles } from "../utils";

interface ChatInputProps {
	isConnected: boolean;
	isLoading: boolean;
	fileContexts: FileContext[];
	onSend: (content: string) => void;
	onFilesUploaded: (files: FileContext[]) => void;
	onError: (error: string) => void;
}

export function ChatInput({
	isConnected,
	isLoading,
	fileContexts,
	onSend,
	onFilesUploaded,
	onError,
}: ChatInputProps) {
	const [inputValue, setInputValue] = useState("");
	const [isUploading, setIsUploading] = useState(false);
	const fileInputRef = useRef<HTMLInputElement>(null);

	const handleSubmit = (e: FormEvent) => {
		e.preventDefault();
		if (!inputValue.trim() || !isConnected || isLoading) return;

		onSend(inputValue);
		setInputValue("");
	};

	const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
		const files = e.target.files;
		if (!files || files.length === 0) return;

		setIsUploading(true);

		const result = await uploadFiles(files);

		if (result.success && result.files) {
			onFilesUploaded([
				...fileContexts,
				...result.files.map((f) => ({
					name: f.name,
					content: f.content,
					size: f.size,
				})),
			]);
		} else if (result.error) {
			onError(result.error);
		}

		setIsUploading(false);
		if (fileInputRef.current) {
			fileInputRef.current.value = "";
		}
	};

	const isDisabled = !isConnected || isLoading;

	return (
		<form onSubmit={handleSubmit} className="flex items-center gap-3">
			<input
				type="file"
				ref={fileInputRef}
				onChange={handleFileChange}
				className="hidden"
				multiple
				accept=".txt,.md,.json,.js,.ts,.jsx,.tsx,.py,.rs,.go,.java,.c,.cpp,.h,.hpp,.css,.html,.xml,.yaml,.yml,.toml,.ini,.cfg,.conf,.sh,.bat,.ps1,.sql"
			/>

			{/* 上传按钮 */}
			<button
				type="button"
				onClick={() => fileInputRef.current?.click()}
				disabled={isDisabled || isUploading}
				className="p-3 rounded-xl hover:bg-gray-100 text-gray-500 hover:text-primary-600 transition-all disabled:opacity-50 disabled:cursor-not-allowed active:scale-95"
				title="上传文件"
			>
				{isUploading ? (
					<Loader2 className="w-5 h-5 animate-spin text-primary-500" />
				) : (
					<FileUp className="w-5 h-5" />
				)}
			</button>

			{/* 输入框 */}
			<div className="flex-1 relative">
				<input
					type="text"
					value={inputValue}
					onChange={(e) => setInputValue(e.target.value)}
					placeholder={isLoading ? "AI 正在思考中..." : "输入消息..."}
					className="w-full bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-[15px] focus:outline-none focus:ring-2 focus:ring-primary-500/20 focus:border-primary-500 transition-all disabled:bg-gray-100 disabled:cursor-not-allowed placeholder:text-gray-400 shadow-sm"
					disabled={isDisabled}
				/>
			</div>

			{/* 发送按钮 */}
			<button
				type="submit"
				className="p-3 rounded-xl bg-primary-600 hover:bg-primary-700 disabled:bg-gray-300 disabled:cursor-not-allowed transition-all shadow-md shadow-primary-200 disabled:shadow-none text-white active:scale-95"
				disabled={!inputValue.trim() || isDisabled}
				title="发送"
			>
				{isLoading ? (
					<Loader2 className="w-5 h-5 animate-spin" />
				) : (
					<Send className="w-5 h-5" />
				)}
			</button>
		</form>
	);
}
