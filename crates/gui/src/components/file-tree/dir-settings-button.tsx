import { useCallback, useRef, useState } from "react";
import { ThreeDotsIcon } from "../../icons/three-dots";
import { IconButton } from "../icon-button";
import { Popper } from "../popper";
import { MenuList } from "../menu/menu-list";
import { MenuItem } from "../menu/menu-item";
import { FSNode } from "../../types";
import { CreateRequestModal } from "../create-request/create-request";

interface Props {
    node: FSNode;
    path: string;
}

export function DirSettingsButton(props: Props) {
    const [open, setOpen] = useState(false);
    const [createRequestModal, setCreateRequestModal] = useState(false);
    const buttonRef = useRef<HTMLButtonElement>(null);

    const createNewRequest = useCallback(() => {
        console.log(props.path);
        setCreateRequestModal(true);
    }, [props.path]);

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
            <Popper
                open={open}
                anchor={buttonRef.current}
                onClose={() => {
                    setOpen(false);
                }}
            >
                <MenuList>
                    <MenuItem onClick={createNewRequest}>New Request</MenuItem>
                    <MenuItem>New Folder</MenuItem>
                    <MenuItem>Delete</MenuItem>
                </MenuList>
            </Popper>
            <CreateRequestModal
                path={props.path}
                open={createRequestModal}
                onClose={() => setCreateRequestModal(false)}
            />
        </>
    );
}