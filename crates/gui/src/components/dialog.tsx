import { ReactNode } from "react";
import { Modal } from "./modal";

interface DialogProps {
    open: boolean;
    onClose?: () => void;
    children: ReactNode | ReactNode[];
}

export function Dialog(props: DialogProps) {
    return (
        <Modal
            open={props.open}
            onClose={props.onClose}
        >
            <div
                onMouseDown={(e) => e.stopPropagation()}
                style={{
                    position: "absolute",
                    top: "50%",
                    left: "50%",
                    transform: "translate(-50%, -50%)",
                    backgroundColor: "var(--color-bg-elevated)",
                    borderRadius: 7
                }}
            >
                {props.children}
            </div>
        </Modal>
    )
}