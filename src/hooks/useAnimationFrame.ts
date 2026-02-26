import { useCallback, useEffect, useRef, useState } from 'react';

export const useAnimationFrame = (args: {
	callback: FrameRequestCallback;
	startOnMount?: boolean;
}) => {
	const frameRequestId = useRef<number | null>(null);
	const [isStarted, setIsStarted] = useState<boolean>(!!args.startOnMount);

	const handleAnimationFrameRequest: FrameRequestCallback = useCallback(
		timestamp => {
			args.callback(timestamp);
			frameRequestId.current = window.requestAnimationFrame(handleAnimationFrameRequest);
		},
		[args.callback]
	);

	const startAnimationFrame = useCallback(() => {
		setIsStarted(true);
		if (frameRequestId.current === null)
			frameRequestId.current = window.requestAnimationFrame(handleAnimationFrameRequest);
	}, []);

	const stopAnimationFrame = useCallback(() => {
		setIsStarted(false);
		if (frameRequestId.current !== null) window.cancelAnimationFrame(frameRequestId.current);
		frameRequestId.current = null;
	}, []);

	const toggle = useCallback(
		(start?: boolean) => {
			const shouldStart = start ?? !isStarted;
			shouldStart ? startAnimationFrame() : stopAnimationFrame();
		},
		[isStarted]
	);

	useEffect(() => {
		if (args.startOnMount) startAnimationFrame();
		return stopAnimationFrame
	}, [args.callback]);

	return { start: startAnimationFrame, cancel: stopAnimationFrame, toggle, isStarted };
};
