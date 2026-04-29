import { useTabs } from "../../store/use-tabs";
import { HeaderTab } from "./tab";

export function HeaderTabs() {
    const { tabs } = useTabs();

    return (
        <div
            style={{
                display: "flex",
                flexDirection: "row",
                alignItems: "center",
                gap: 2,
            }}
        >
            {tabs.map((tab) => (
                <HeaderTab
                    tab={tab}
                    key={tab.path}
                />
            ))}
        </div>
    )
}