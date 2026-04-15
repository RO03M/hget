import { useState, useRef, useEffect } from "react";
import styles from "./method-select.module.css";

const HTTP_METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

interface Props {
    value: string;
    onChange: (method: string) => void;
}

export function MethodSelect({ value, onChange }: Props) {
    const [open, setOpen] = useState(false);
    const [isCustom, setIsCustom] = useState(false);
    const [customValue, setCustomValue] = useState("");
    const containerRef = useRef<HTMLDivElement>(null);
    const customInputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        function handleOutsideClick(e: MouseEvent) {
            if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
                setOpen(false);
            }
        }
        document.addEventListener("mousedown", handleOutsideClick);
        return () => document.removeEventListener("mousedown", handleOutsideClick);
    }, []);

    useEffect(() => {
        if (isCustom) customInputRef.current?.focus();
    }, [isCustom]);

    function selectMethod(method: string) {
        onChange(method);
        setOpen(false);
    }

    function openCustomInput() {
        setCustomValue("");
        setIsCustom(true);
        setOpen(false);
    }

    function commitCustom() {
        const trimmed = customValue.trim().toUpperCase();
        if (trimmed) onChange(trimmed);
        setIsCustom(false);
    }

    function handleCustomKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
        if (e.key === "Enter") commitCustom();
        if (e.key === "Escape") setIsCustom(false);
    }

    const methodKey = value.toLowerCase() as keyof typeof styles;
    const triggerColor = styles[methodKey] ?? styles.custom;

    if (isCustom) {
        return (
            <input
                ref={customInputRef}
                className={`${styles.trigger} ${styles.customInput}`}
                value={customValue}
                onChange={(e) => setCustomValue(e.target.value)}
                onKeyDown={handleCustomKeyDown}
                onBlur={commitCustom}
                placeholder="CUSTOM"
                spellCheck={false}
            />
        );
    }

    return (
        <div ref={containerRef} className={styles.container}>
            <button
                type="button"
                className={`${styles.trigger} ${triggerColor}`}
                onClick={() => setOpen((o) => !o)}
            >
                {value}
                <span className={styles.caret}>▾</span>
            </button>

            {open && (
                <div className={styles.dropdown}>
                    {HTTP_METHODS.map((method) => (
                        <button
                            key={method}
                            type="button"
                            className={`${styles.option} ${styles[method.toLowerCase() as keyof typeof styles]}`}
                            onClick={() => selectMethod(method)}
                        >
                            {method}
                        </button>
                    ))}
                    <div className={styles.divider} />
                    <button
                        type="button"
                        className={`${styles.option} ${styles.addCustom}`}
                        onClick={openCustomInput}
                    >
                        + Add Custom
                    </button>
                </div>
            )}
        </div>
    );
}
