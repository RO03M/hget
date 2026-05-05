import { ReactNode, useRef } from "react";
import { createPortal } from "react-dom";

interface Props extends React.ComponentProps<"div"> {
    open: boolean;
    children: ReactNode;
    anchor: HTMLElement | null;
    onClose: () => void;
}

export function Popper(props: Props) {
    const containerRef = useRef<HTMLDivElement>(null);

    if (!props.open) {
        return null;
    }

    if (!document || !document.body) {
        console.warn("[popover]: missing document.body");
        return null;
    }

    if (!props.anchor) {
        return null;
    }

    return createPortal(
        <div
            aria-description="popper presentation"
            onClick={(e) => {
                e.stopPropagation();
                props.onClose();
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
                ref={containerRef}
                {...props}
                style={{
                    position: "absolute",
                    top: props.anchor.getBoundingClientRect().bottom + window.scrollY,
                    left: props.anchor.getBoundingClientRect().left + window.scrollX,
                    zIndex: 1000,
                }}
            >
                {props.children}
            </div>
        </div>,
        document.body
    );
}