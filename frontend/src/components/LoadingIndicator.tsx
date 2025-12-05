import { Sparkles } from "lucide-react";

export function LoadingIndicator() {
	return (
		<div className="flex justify-start my-2">
			<div className="flex gap-3">
				<div className="flex-shrink-0 w-8 h-8 bg-gradient-to-br from-violet-500 to-indigo-600 rounded-lg flex items-center justify-center shadow-sm">
					<Sparkles className="w-4 h-4 text-white" />
				</div>
				<div className="bg-white border border-gray-100 rounded-2xl rounded-tl-sm px-4 py-3 shadow-sm">
					<div className="flex items-center gap-2">
						<div className="flex gap-1">
							<div
								className="w-2 h-2 bg-violet-400 rounded-full animate-bounce"
								style={{ animationDelay: "0ms" }}
							/>
							<div
								className="w-2 h-2 bg-violet-400 rounded-full animate-bounce"
								style={{ animationDelay: "150ms" }}
							/>
							<div
								className="w-2 h-2 bg-violet-400 rounded-full animate-bounce"
								style={{ animationDelay: "300ms" }}
							/>
						</div>
						<span className="text-sm text-gray-500">正在思考...</span>
					</div>
				</div>
			</div>
		</div>
	);
}
