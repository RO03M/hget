import { useRef, useState } from "react";
import { ThreeDotsIcon } from "../../icons/three-dots";
import { IconButton } from "../icon-button";
import { FSNode } from "../../types";
import { FolderContextMenu } from "./folder-context-menu";

interface Props {
    node: FSNode;
    path: string;
}

export function DirSettingsButton(props: Props) {
    const [open, setOpen] = useState(false);
    const buttonRef = useRef<HTMLButtonElement>(null);

    return (
        <>
            <IconButton
                size={20}
                ref={buttonRef}
                onClick={(event) => {
                    event.stopPropagation();
                    setOpen(true);
                }}
            >
                <ThreeDotsIcon size={16}/>
            </IconButton>
            <FolderContextMenu
                open={open}
                path={props.path}
                onClose={() => setOpen(false)}
                anchor={buttonRef.current}
            />
        </>
    );
}