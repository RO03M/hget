import { ReactNode } from "react";
import { createPortal } from "react-dom";

interface ModalProps {
    children: ReactNode | ReactNode[];
    open: boolean;
    onClose?: () => void;
}

export function Modal(props: ModalProps) {
    if (!props.open) {
        return null;
    }

    return createPortal(
        <div
            onMouseDown={(e) => {
                e.stopPropagation();
                props.onClose?.();
            }}
            onClick={(e) => {
                e.stopPropagation();
            }}
            aria-description="modal presentation"
            style={{
                backgroundColor: "rgba(0, 0, 0, 0.1)",
                width: "100%",
                height: "100%",
                position: "fixed",
                top: 0,
                left: 0,
                zIndex: 1200
            }}
        >
            {props.children}
        </div>,
        document.body
    );
}