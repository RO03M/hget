import { CloseIcon } from "../../icons/close";
import { IconButton } from "../icon-button";
import styles from "./create-request.module.css";

export function NewRequestHeader() {
    return (
        <div
            className={styles.header}
        >
            <span>New Request</span>
            <IconButton>
                <CloseIcon size={20} />
            </IconButton>
        </div>
    )
}