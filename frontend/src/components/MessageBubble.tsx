import { Brain, Sparkles, User } from "lucide-react";
import ReactMarkdown, { type Components } from "react-markdown";
import remarkGfm from "remark-gfm";
import type { Message } from "../types";

interface MessageBubbleProps {
	message: Message;
}

// Markdown 渲染组件样式
const markdownComponents: Partial<Components> = {
	// 代码块
	code: ({ className, children, ...props }) => {
		const match = /language-(\w+)/.exec(className || "");
		const isInline = !match;

		if (isInline) {
			return (
				<code
					className="bg-gray-100 text-pink-600 px-1.5 py-0.5 rounded text-sm font-mono"
					{...props}
				>
					{children}
				</code>
			);
		}

		return (
			<div className="my-2">
				<div className="bg-gray-800 text-gray-100 rounded-lg overflow-hidden">
					<div className="flex items-center justify-between px-4 py-2 bg-gray-700 text-xs text-gray-300">
						<span>{match[1]}</span>
					</div>
					<pre className="p-4 overflow-x-auto">
						<code className="text-sm font-mono" {...props}>
							{children}
						</code>
					</pre>
				</div>
			</div>
		);
	},
	// 段落
	p: ({ children }) => <p className="mb-2 last:mb-0">{children}</p>,
	// 链接
	a: ({ href, children }) => (
		<a
			href={href}
			target="_blank"
			rel="noopener noreferrer"
			className="text-violet-600 hover:text-violet-800 underline"
		>
			{children}
		</a>
	),
	// 列表
	ul: ({ children }) => (
		<ul className="list-disc list-inside my-2 space-y-1">{children}</ul>
	),
	ol: ({ children }) => (
		<ol className="list-decimal list-inside my-2 space-y-1">{children}</ol>
	),
	// 引用
	blockquote: ({ children }) => (
		<blockquote className="border-l-4 border-violet-300 pl-4 my-2 text-gray-600 italic">
			{children}
		</blockquote>
	),
	// 标题
	h1: ({ children }) => (
		<h1 className="text-xl font-bold mt-4 mb-2">{children}</h1>
	),
	h2: ({ children }) => (
		<h2 className="text-lg font-bold mt-3 mb-2">{children}</h2>
	),
	h3: ({ children }) => (
		<h3 className="text-base font-bold mt-2 mb-1">{children}</h3>
	),
	// 表格
	table: ({ children }) => (
		<div className="overflow-x-auto my-2">
			<table className="min-w-full border-collapse border border-gray-300">
				{children}
			</table>
		</div>
	),
	th: ({ children }) => (
		<th className="border border-gray-300 bg-gray-100 px-3 py-2 text-left font-semibold">
			{children}
		</th>
	),
	td: ({ children }) => (
		<td className="border border-gray-300 px-3 py-2">{children}</td>
	),
	// 水平线
	hr: () => <hr className="my-4 border-gray-200" />,
	// 强调
	strong: ({ children }) => (
		<strong className="font-semibold">{children}</strong>
	),
	em: ({ children }) => <em className="italic">{children}</em>,
};

export function MessageBubble({ message }: MessageBubbleProps) {
	const { type, content, model } = message;

	// 系统消息
	if (type === "system") {
		return (
			<div className="flex justify-center my-3">
				<div className="bg-gray-100 text-gray-600 text-xs px-4 py-1.5 rounded-full">
					{content}
				</div>
			</div>
		);
	}

	// 思考过程消息
	if (type === "thinking") {
		return (
			<div className="flex justify-start my-2">
				<div className="max-w-[85%] md:max-w-[75%]">
					<div className="flex items-center gap-2 mb-1.5">
						<Brain className="w-4 h-4 text-violet-500" />
						<span className="text-xs font-medium text-violet-600">
							思考过程
						</span>
					</div>
					<div className="bg-violet-50 border border-violet-100 rounded-2xl rounded-tl-sm px-4 py-3">
						<div className="text-sm text-violet-800 leading-relaxed">
							<ReactMarkdown
								remarkPlugins={[remarkGfm]}
								components={markdownComponents}
							>
								{content}
							</ReactMarkdown>
						</div>
					</div>
				</div>
			</div>
		);
	}

	const isUser = type === "user";

	return (
		<div className={`flex ${isUser ? "justify-end" : "justify-start"} my-2`}>
			<div
				className={`max-w-[85%] md:max-w-[75%] ${isUser ? "" : "flex gap-3"}`}
			>
				{/* AI 头像 */}
				{!isUser && (
					<div className="flex-shrink-0 w-8 h-8 bg-gradient-to-br from-violet-500 to-indigo-600 rounded-lg flex items-center justify-center shadow-sm">
						<Sparkles className="w-4 h-4 text-white" />
					</div>
				)}

				<div className="flex flex-col">
					{/* 模型标识 */}
					{!isUser && model && (
						<span className="text-xs text-gray-400 mb-1 ml-1">{model}</span>
					)}

					{/* 消息气泡 */}
					<div
						className={`px-4 py-3 ${
							isUser
								? "bg-gradient-to-br from-violet-500 to-indigo-600 text-white rounded-2xl rounded-tr-sm shadow-md shadow-violet-200"
								: "bg-white text-gray-800 rounded-2xl rounded-tl-sm border border-gray-100 shadow-sm"
						}`}
					>
						{isUser ? (
							<p className="whitespace-pre-wrap text-[15px] leading-relaxed">
								{content}
							</p>
						) : (
							<div className="text-[15px] leading-relaxed prose prose-sm max-w-none">
								<ReactMarkdown
									remarkPlugins={[remarkGfm]}
									components={markdownComponents}
								>
									{content}
								</ReactMarkdown>
							</div>
						)}
					</div>
				</div>

				{/* 用户头像 */}
				{isUser && (
					<div className="flex-shrink-0 w-8 h-8 bg-gray-200 rounded-lg flex items-center justify-center ml-3">
						<User className="w-4 h-4 text-gray-600" />
					</div>
				)}
			</div>
		</div>
	);
}
