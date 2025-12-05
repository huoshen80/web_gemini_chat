import { useEffect, useRef } from "react";
import {
	ChatInput,
	EmptyState,
	ErrorBanner,
	FilePreview,
	Header,
	LoadingIndicator,
	MessageBubble,
	ModelSelector,
} from "./components";
import { useWebSocket } from "./hooks";

function App() {
	const {
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
		isConnected,
	} = useWebSocket();

	const messagesEndRef = useRef<HTMLDivElement>(null);

	// 自动滚动到底部
	useEffect(() => {
		messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
	}, [messages, isLoading]);

	// 移除单个文件
	const handleRemoveFile = (index: number) => {
		const newFiles = fileContexts.filter((_, i) => i !== index);
		if (newFiles.length > 0) {
			setContextFiles(newFiles);
		} else {
			clearContextFiles();
		}
	};

	return (
		<div className="h-screen w-full flex flex-col bg-gradient-to-br from-slate-50 via-white to-violet-50">
			{/* 顶部导航 */}
			<Header
				connectionStatus={connectionStatus}
				onClearChat={clearChat}
				hasMessages={messages.length > 0}
			/>

			{/* 错误提示 */}
			<ErrorBanner error={error} onClose={clearError} />

			{/* 消息区域 */}
			<main className="flex-1 overflow-y-auto">
				<div className="max-w-4xl mx-auto px-4 py-6">
					{messages.length === 0 && !isLoading ? (
						<EmptyState />
					) : (
						<div className="flex flex-col">
							{messages.map((msg) => (
								<MessageBubble key={msg.id} message={msg} />
							))}
							{isLoading && <LoadingIndicator />}
							<div ref={messagesEndRef} />
						</div>
					)}
				</div>
			</main>

			{/* 底部输入区域 - 包含模型切换和文件预览 */}
			<footer className="bg-white/90 backdrop-blur-lg border-t border-gray-200/50 shadow-lg shadow-gray-100/50">
				<div className="max-w-4xl mx-auto px-4 py-3">
					{/* 文件预览 - 在输入框上方 */}
					<FilePreview
						files={fileContexts}
						onRemove={handleRemoveFile}
						onClear={clearContextFiles}
					/>

					{/* 输入行：模型选择器 + 输入框 */}
					<div className="flex items-end gap-3">
						{/* 模型选择器 */}
						<ModelSelector
							currentModel={currentModel}
							onModelChange={switchModel}
						/>

						{/* 输入框 */}
						<div className="flex-1">
							<ChatInput
								isConnected={isConnected}
								isLoading={isLoading}
								fileContexts={fileContexts}
								onSend={sendChat}
								onFilesUploaded={setContextFiles}
								onError={(err) => console.error(err)}
							/>
						</div>
					</div>
				</div>
			</footer>
		</div>
	);
}

export default App;
