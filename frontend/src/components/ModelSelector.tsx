import { ChevronUp, Sparkles, Zap } from "lucide-react";
import { useState } from "react";
import type { ModelType } from "../types";
import { MODELS } from "../types";

interface ModelSelectorProps {
	currentModel: ModelType;
	onModelChange: (model: ModelType) => void;
}

export function ModelSelector({
	currentModel,
	onModelChange,
}: ModelSelectorProps) {
	const [showMenu, setShowMenu] = useState(false);

	const currentModelInfo = MODELS.find((m) => m.id === currentModel);

	return (
		<div className="relative">
			<button
				type="button"
				onClick={() => setShowMenu(!showMenu)}
				className="flex items-center gap-2 px-3 py-2 bg-gray-50 hover:bg-gray-100 rounded-xl transition-colors text-sm font-medium text-gray-600 border border-gray-200"
			>
				{currentModel === "flash" ? (
					<Zap className="w-4 h-4 text-amber-500" />
				) : (
					<Sparkles className="w-4 h-4 text-violet-500" />
				)}
				<span className="hidden sm:inline">{currentModelInfo?.name}</span>
				<ChevronUp
					className={`w-4 h-4 text-gray-400 transition-transform ${showMenu ? "" : "rotate-180"}`}
				/>
			</button>

			{showMenu && (
				<>
					<button
						type="button"
						className="fixed inset-0 z-10 cursor-default"
						onClick={() => setShowMenu(false)}
					/>
					<div className="absolute bottom-full left-0 mb-2 w-64 bg-white rounded-xl shadow-xl border border-gray-100 py-1 z-20">
						{MODELS.map((model) => (
							<button
								type="button"
								key={model.id}
								onClick={() => {
									onModelChange(model.id);
									setShowMenu(false);
								}}
								className={`w-full px-4 py-2.5 flex items-center gap-3 hover:bg-gray-50 transition-colors ${
									currentModel === model.id ? "bg-violet-50" : ""
								}`}
							>
								{model.id === "flash" ? (
									<Zap className="w-4 h-4 text-amber-500" />
								) : (
									<Sparkles className="w-4 h-4 text-violet-500" />
								)}
								<div className="text-left flex-1">
									<div className="font-medium text-gray-800">{model.name}</div>
									<div className="text-xs text-gray-500">
										{model.description}
									</div>
								</div>
								{currentModel === model.id && (
									<div className="w-2 h-2 rounded-full bg-violet-500" />
								)}
							</button>
						))}
					</div>
				</>
			)}
		</div>
	);
}
