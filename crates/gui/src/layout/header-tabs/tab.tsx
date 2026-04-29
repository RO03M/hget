import { IconButton } from "../../components/icon-button";
import { CloseIcon } from "../../icons/close";
import { Tab } from "../../store/types";
import { useTabs } from "../../store/use-tabs";
import styles from "./index.module.css";

interface Props {
    tab: Tab;
}

export function HeaderTab(props: Props) {
    const { focus, close } = useTabs();

    return (
        <div
            className={`${styles.tab} ${props.tab.active ? styles.active : ""}`}
            onClick={() => focus(props.tab.path)}
        >
            <div>{props.tab.path}</div>
            <IconButton
                onClick={(e) => { e.stopPropagation(); close(props.tab.path); }}
            >
                <CloseIcon size={12} />
            </IconButton>
        </div>
    )
}