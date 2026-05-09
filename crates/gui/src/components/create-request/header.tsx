import { CloseIcon } from "../../icons/close";
import { IconButton } from "../icon-button";
import styles from "./create-request.module.css";

interface Props {
    onClose: () => void;
}

export function NewRequestHeader(props: Props) {
    return (
        <div
            className={styles.header}
        >
            <span>New Request</span>
            <IconButton onClick={props.onClose}>
                <CloseIcon size={20} />
            </IconButton>
        </div>
    )
}