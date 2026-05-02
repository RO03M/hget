import { useState } from "react";
import { HttpResponse } from "../../../types"
import { Tabs } from "../../../components/tabs";
import { Tab } from "../../../components/tab";
import { HeadersTab } from "./headers-tab";
import { ResponseTab } from "./response-tab";

interface Props {
    response: HttpResponse | null;
}

type TabValue = "response" | "headers";

export function ResponseContainer(props: Props) {
    const [currentTab, setCurrentTab] = useState<TabValue>("response");

    if (!props.response) {
        return <div>send a request first!</div>
    }

    return (
        <div
            style={{ height: "100%" }}
        >
            <Tabs
                value={currentTab}
                onChange={(value) => setCurrentTab(value)}
            >
                <Tab label="Response" value={"response"} />
                <Tab label="Headers" value={"headers"} />
            </Tabs>

            <div
                style={{
                    overflowY: "scroll",
                    height: "80vh"
                }}
            >
                {currentTab == "response" && (
                    <ResponseTab
                        response={props.response}
                    />
                )}
                {currentTab == "headers" && (
                    <HeadersTab
                        headers={props.response.headers}
                    />
                )}
            </div>
        </div>
    )
}