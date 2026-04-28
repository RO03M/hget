import { IconSquaredProps } from "./types";

export function ArrowIcon(props: IconSquaredProps) {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg" 
            width={props.size}
            height={(props.size ?? 0) * 2}
            viewBox="0 0 12 24"
            style={{ transform: `rotate(${props.rotation ?? 0}deg)` }}
        >
            <path fill="currentColor" fillRule="evenodd" d="M10.157 12.711L4.5 18.368l-1.414-1.414l4.95-4.95l-4.95-4.95L4.5 5.64l5.657 5.657a1 1 0 0 1 0 1.414"></path>
        </svg>
    );
}