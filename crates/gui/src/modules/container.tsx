import { HeaderTabs } from "../layout/header-tabs/tabs";
import { HttpPageContainer } from "./http-page";

export function Container() {
    return (
        <div
            style={{
                height: "100vh",
                display: "flex",
                flexDirection: "column",
            }}
        >
            <HeaderTabs />
            <HttpPageContainer />
            {/* <div
                style={{
                    backgroundColor: "red",
                    flex: 1,
                    overflowY: "scroll"
                }}
            >
                {Array.from(Array(100).keys()).map(() => (
                    <p>fill</p>
                ))}
            </div> */}
        </div>
    );
}