import { File, X } from "lucide-react";
import type { FileContext } from "../types";
import { formatFileSize } from "../utils";

interface FilePreviewProps {
	files: FileContext[];
	onRemove: (index: number) => void;
	onClear: () => void;
}

export function FilePreview({ files, onRemove, onClear }: FilePreviewProps) {
	if (files.length === 0) return null;

	return (
		<div className="mb-3 p-3 bg-gray-50/80 rounded-xl border border-gray-100">
			<div className="flex items-center justify-between mb-2">
				<span className="text-xs font-medium text-gray-500">
					📎 已附加 {files.length} 个文件
				</span>
				<button
					type="button"
					onClick={onClear}
					className="text-xs text-gray-400 hover:text-red-500 transition-colors px-2 py-0.5 rounded hover:bg-red-50"
				>
					清除全部
				</button>
			</div>
			<div className="flex flex-wrap gap-2">
				{files.map((file, index) => (
					<div
						key={`${file.name}-${index}`}
						className="flex items-center gap-2 bg-white rounded-lg px-2.5 py-1.5 border border-gray-200 shadow-sm group hover:border-violet-200 transition-colors"
					>
						<File className="w-3.5 h-3.5 text-violet-500" />
						<span className="text-xs text-gray-700 max-w-24 truncate font-medium">
							{file.name}
						</span>
						<span className="text-xs text-gray-400">
							{formatFileSize(file.size)}
						</span>
						<button
							type="button"
							onClick={() => onRemove(index)}
							className="text-gray-300 hover:text-red-500 transition-colors p-0.5 rounded hover:bg-red-50"
						>
							<X className="w-3.5 h-3.5" />
						</button>
					</div>
				))}
			</div>
		</div>
	);
}
