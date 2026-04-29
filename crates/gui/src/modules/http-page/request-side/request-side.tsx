import { useState } from "react";
import { useWatch } from "react-hook-form";
import { ParamsTab } from "./tabs/params-tab";
import { HeadersTab } from "./tabs/headers-tab";
import { BodyTab } from "./tabs/body-tab";
import { AuthTab } from "./tabs/auth-tab";
import { KeyValueRow } from "./types";
import styles from "./request-side.module.css";

type Tab = "params" | "body" | "headers" | "auth";

function countActive(rows: KeyValueRow[]): number {
    return rows.filter((r) => r.enabled && (r.name || r.value)).length;
}

export function RequestSide() {
    const [activeTab, setActiveTab] = useState<Tab>("params");

    const params  = useWatch({ name: "params"  }) as KeyValueRow[];
    const headers = useWatch({ name: "headers" }) as KeyValueRow[];

    const tabs: { id: Tab; label: string; badge?: number }[] = [
        { id: "params",  label: "Params",  badge: countActive(params) },
        { id: "body",    label: "Body" },
        { id: "headers", label: "Headers", badge: countActive(headers) },
        { id: "auth",    label: "Auth" },
    ];

    return (
        <div className={styles.container}>
            <div className={styles.tabBar}>
                {tabs.map((tab) => (
                    <button
                        key={tab.id}
                        type="button"
                        className={`${styles.tabBtn} ${activeTab === tab.id ? styles.tabBtnActive : ""}`}
                        onClick={() => setActiveTab(tab.id)}
                    >
                        {tab.label}
                        {!!tab.badge && (
                            <span className={styles.badge}>{tab.badge}</span>
                        )}
                    </button>
                ))}
            </div>

            <div className={styles.tabContent}>
                {activeTab === "params"  && <ParamsTab />}
                {activeTab === "body"    && <BodyTab />}
                {activeTab === "headers" && <HeadersTab />}
                {activeTab === "auth"    && <AuthTab />}
            </div>
        </div>
    );
}
