import { ReactNode, useEffect, useRef, useState } from "react";
import styles from "./split-pane.module.css";

interface SplitPaneProps {
    children: [ReactNode, ReactNode];
    initialLeftWidth?: number;
    min?: number,
    max?: number
}

export function SplitPane(props: SplitPaneProps) {
    const { initialLeftWidth = 300, min = 150, max = 800 } = props;

    const containerRef = useRef<HTMLDivElement>(null);
    const [leftWidth, setLeftWidth] = useState(initialLeftWidth);
    const [dragging, setDragging] = useState(false);

    useEffect(() => {
        const handleMouseMove = (e: MouseEvent) => {
            if (!dragging || !containerRef.current) {
                return;
            }

            const containerLeft = containerRef.current.getBoundingClientRect().left;

            let newWidth = e.clientX - containerLeft;
            newWidth = Math.max(min, Math.min(newWidth, max));

            setLeftWidth(newWidth);
        };

        const handleMouseUp = () => {
            setDragging(false);
            document.body.style.userSelect = "";
        };

        window.addEventListener("mousemove", handleMouseMove);
        window.addEventListener("mouseup", handleMouseUp);

        return () => {
            window.removeEventListener("mousemove", handleMouseMove);
            window.removeEventListener("mouseup", handleMouseUp);
        };
    }, [dragging, min, max]);

    const startDragging = () => {
        setDragging(true);
        document.body.style.userSelect = "none";
    };

    const [left, right] = props.children;

    return (
        <div
            ref={containerRef}
            style={{
                display: "flex",
                width: "100%",
                height: "100%"
            }}
        >
            <div
                style={{
                    width: leftWidth,
                    minWidth: min,
                    overflow: "auto",
                }}
            >
                {left}
            </div>

            <div
                onMouseDown={startDragging}
                className={styles['divider-container']}
            >
                <div
                    style={{
                        width: 1,
                        background: "#ccc",
                        height: "100%"
                    }}
                />
            </div>

            <div
                style={{
                    flex: 1,
                    overflow: "auto",
                }}
            >
                {right}
            </div>
        </div>
    )
}