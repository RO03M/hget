import { useCallback, useEffect, type RefObject } from "react";

export function useClickOutside(
	ref: RefObject<HTMLElement | null>,
	callback: () => void
) {
	const onClick = useCallback(
		(event: MouseEvent) => {
			if (ref.current && !ref.current.contains(event.target as HTMLElement)) {
				callback();
			}
		},
		[ref, callback]
	);

	useEffect(() => {
		document.addEventListener("click", onClick);

		return () => {
			document.removeEventListener("click", onClick);
		};
	});
}