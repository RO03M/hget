import { ReactNode, useMemo, useRef } from "react";
import { createPortal } from "react-dom";

interface Props extends React.ComponentProps<"div"> {
    open: boolean;
    children: ReactNode;
    anchor?: HTMLElement | null;
    anchorPosition?: { top: number, left: number };
    onClose: () => void;
}

export function Popper(props: Props) {
    const { anchor, anchorPosition, onClose, open, children, ...rest } = props;

    const containerRef = useRef<HTMLDivElement>(null);

    const coords = useMemo(() => {
        if (anchor) {
            return {
                top: anchor.getBoundingClientRect().bottom + window.scrollY,
                left: anchor.getBoundingClientRect().left + window.scrollX,
            };
        }

        if (anchorPosition) {
            return {
                top: anchorPosition.top,
                left: anchorPosition.left,
            };
        }

        return { top: 0, left: 0 };
    }, [anchor, anchorPosition]);

    if (!open) {
        return null;
    }

    if (!document || !document.body) {
        console.warn("[popover]: missing document.body");
        return null;
    }

    return createPortal(
        <div
            aria-description="popper presentation"
            onClick={(e) => {
                e.stopPropagation();
                onClose();
            }}
            style={{
                position: "fixed",
                width: "100%",
                height: "100%",
                left: 0,
                top: 0,
            }}
        >
            <div
                {...rest}
                ref={containerRef}
                onClick={(e) => {
                    e.stopPropagation();
                    rest.onClick?.(e);
                }}
                style={{
                    position: "absolute",
                    top: coords.top,
                    left: coords.left,
                    zIndex: 1000,
                }}
            >
                {children}
            </div>
        </div>,
        document.body
    );
}