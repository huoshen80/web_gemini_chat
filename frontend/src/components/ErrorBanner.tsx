import { AlertCircle, X } from "lucide-react";

interface ErrorBannerProps {
	error: string;
	onClose: () => void;
}

export function ErrorBanner({ error, onClose }: ErrorBannerProps) {
	if (!error) return null;

	return (
		<div className="bg-red-50 border-b border-red-100 px-4 py-2">
			<div className="max-w-4xl mx-auto flex items-center justify-between">
				<div className="flex items-center gap-2 text-red-600">
					<AlertCircle className="w-4 h-4 flex-shrink-0" />
					<span className="text-sm">{error}</span>
				</div>
				<button
					type="button"
					onClick={onClose}
					className="text-red-400 hover:text-red-600 transition-colors"
				>
					<X className="w-4 h-4" />
				</button>
			</div>
		</div>
	);
}
