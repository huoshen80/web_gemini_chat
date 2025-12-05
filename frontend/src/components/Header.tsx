import { Sparkles, Trash2 } from "lucide-react";
import type { ConnectionStatus } from "../types";

interface HeaderProps {
	connectionStatus: ConnectionStatus;
	onClearChat: () => void;
	hasMessages: boolean;
}

export function Header({
	connectionStatus,
	onClearChat,
	hasMessages,
}: HeaderProps) {
	return (
		<header className="bg-white/80 backdrop-blur-md border-b border-gray-100 sticky top-0 z-10">
			<div className="max-w-4xl mx-auto px-4 py-3 flex items-center justify-between">
				{/* Logo & Title */}
				<div className="flex items-center gap-3">
					<div className="w-9 h-9 bg-gradient-to-br from-violet-500 to-indigo-600 rounded-xl flex items-center justify-center shadow-lg shadow-violet-200">
						<Sparkles className="w-5 h-5 text-white" />
					</div>
					<h1 className="text-lg font-semibold text-gray-800">Web Chat</h1>
				</div>

				{/* Actions & Status */}
				<div className="flex items-center gap-4">
					{/* Clear Chat Button */}
					{hasMessages && (
						<button
							type="button"
							onClick={onClearChat}
							className="flex items-center gap-1.5 px-2.5 py-1.5 text-xs text-gray-500 hover:text-red-500 hover:bg-red-50 rounded-lg transition-colors"
							title="清除聊天记录"
						>
							<Trash2 className="w-3.5 h-3.5" />
							<span className="hidden sm:inline">清除记录</span>
						</button>
					)}

					{/* Connection Status */}
					<div className="flex items-center gap-2">
						<div
							className={`w-2 h-2 rounded-full ${
								connectionStatus === "connected"
									? "bg-emerald-500"
									: connectionStatus === "connecting"
										? "bg-amber-500 animate-pulse"
										: "bg-red-500"
							}`}
						/>
						<span className="text-xs text-gray-500">
							{connectionStatus === "connected"
								? "在线"
								: connectionStatus === "connecting"
									? "连接中"
									: "离线"}
						</span>
					</div>
				</div>
			</div>
		</header>
	);
}
