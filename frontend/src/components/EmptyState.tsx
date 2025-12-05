import { Sparkles } from "lucide-react";

export function EmptyState() {
	return (
		<div className="flex flex-col items-center justify-center h-full text-center py-20">
			<div className="w-20 h-20 bg-gradient-to-br from-violet-500 to-indigo-600 rounded-3xl flex items-center justify-center mb-6 shadow-xl shadow-violet-200">
				<Sparkles className="w-10 h-10 text-white" />
			</div>
			<h2 className="text-2xl font-semibold text-gray-800 mb-3">开始对话</h2>
			<p className="text-gray-500 text-sm max-w-sm leading-relaxed">
				向 Gemini 发送消息开始智能对话
				<br />
				支持上传文件作为对话上下文
			</p>
			<div className="mt-8 flex flex-wrap justify-center gap-2">
				<div className="px-3 py-1.5 bg-gray-100 rounded-full text-xs text-gray-600">
					💡 可以问我任何问题
				</div>
				<div className="px-3 py-1.5 bg-gray-100 rounded-full text-xs text-gray-600">
					📄 支持上传文件
				</div>
				<div className="px-3 py-1.5 bg-gray-100 rounded-full text-xs text-gray-600">
					🔄 可切换模型
				</div>
			</div>
		</div>
	);
}
