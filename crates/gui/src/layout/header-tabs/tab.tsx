import { IconButton } from "../../components/icon-button";
import { CloseIcon } from "../../icons/close";
import { Tab } from "../../store/types";
import { useTabs } from "../../store/use-tabs";

interface Props {
    tab: Tab;
}

export function HeaderTab(props: Props) {
    const { focus, close } = useTabs();

    return (
        <div
            style={{
                display: "flex",
                flexDirection: "row",
                alignItems: "center",
                padding: 4,
                border: "1px solid gray",
                userSelect: "none",
                WebkitUserSelect: "none",
                cursor: "pointer",
                backgroundColor: props.tab.active ? "purple" : "inherit",
            }}
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