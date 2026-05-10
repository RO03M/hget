import { useCallback, useState } from "react";
import { MenuItem } from "../menu/menu-item";
import { MenuList } from "../menu/menu-list";
import { CreateRequestModal } from "../create-request/create-request";
import { Popper } from "../popper";

interface Props {
    path: string;
    open: boolean;
    onClose?: () => void;
    anchor?: HTMLElement | null;
    anchorPosition?: { top: number, left: number };
}

export function FolderContextMenu(props: Props) {
    const { open, path, anchor, anchorPosition, onClose } = props;

    const [createRequestModal, setCreateRequestModal] = useState(false);
    const [createDirModal, setCreateDirModal] = useState(false);

    const createNewRequest = useCallback(() => {
        onClose?.();
        setCreateRequestModal(true);
    }, []);

    const createDir = useCallback(() => {
        onClose?.();
        setCreateDirModal(true);
    }, []);

    return (
        <>
            <Popper
                open={open}
                onClose={() => onClose?.()}
                anchor={anchor}
                anchorPosition={anchorPosition}
            >
                <MenuList>
                    <MenuItem onClick={createNewRequest}>New Request</MenuItem>
                    <MenuItem onClick={createDir}>New Folder</MenuItem>
                    <MenuItem>Delete</MenuItem>
                </MenuList>
            </Popper>
            <CreateRequestModal
                path={path}
                open={createRequestModal}
                onClose={() => setCreateRequestModal(false)}
            />
        </>
    );
}