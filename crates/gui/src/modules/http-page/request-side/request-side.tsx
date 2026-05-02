import { useState } from "react";
import { useWatch } from "react-hook-form";
import { ParamsTab } from "./tabs/params-tab";
import { HeadersTab } from "./tabs/headers-tab";
import { BodyTab } from "./tabs/body-tab";
import { AuthTab } from "./tabs/auth-tab";
import { KeyValueRow } from "./types";
import styles from "./request-side.module.css";
import { Tabs } from "../../../components/tabs";
import { Tab } from "../../../components/tab";

type TabId = "params" | "body" | "headers" | "auth";

function countActive(rows: KeyValueRow[]): number {
    return rows.filter((r) => r.enabled && (r.name || r.value)).length;
}

export function RequestSide() {
    const [activeTab, setActiveTab] = useState<TabId>("params");

    const params  = useWatch({ name: "params"  }) as KeyValueRow[];
    const headers = useWatch({ name: "headers" }) as KeyValueRow[];

    const tabs: { id: TabId; label: string; badge?: number }[] = [
        { id: "params",  label: "Params",  badge: countActive(params) },
        { id: "body",    label: "Body" },
        { id: "headers", label: "Headers", badge: countActive(headers) },
        { id: "auth",    label: "Auth" },
    ];

    return (
        <div className={styles.container}>
            <Tabs
                onChange={setActiveTab}
                value={activeTab}
                className={styles.tabBar}
            >
                {tabs.map((tab) => (
                    <Tab
                        label={tab.label}
                        key={tab.id}
                        value={tab.id}
                    />
                ))}
            </Tabs>

            <div className={styles.tabContent}>
                {activeTab === "params"  && <ParamsTab />}
                {activeTab === "body"    && <BodyTab />}
                {activeTab === "headers" && <HeadersTab />}
                {activeTab === "auth"    && <AuthTab />}
            </div>
        </div>
    );
}
