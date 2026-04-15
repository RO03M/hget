import { useFormContext, useWatch } from "react-hook-form";
import { HttpRequest } from "../types";
import styles from "./auth-tab.module.css";

export function AuthTab() {
    const { register } = useFormContext<HttpRequest>();
    const authType = useWatch<HttpRequest, "auth.type">({ name: "auth.type" });

    return (
        <div className={styles.container}>
            <div className={styles.field}>
                <label className={styles.label}>Type</label>
                <select className={styles.select} {...register("auth.type")}>
                    <option value="none">None</option>
                    <option value="bearer">Bearer Token</option>
                    <option value="basic">Basic Auth</option>
                </select>
            </div>

            {authType === "bearer" && (
                <div className={styles.field}>
                    <label className={styles.label}>Token</label>
                    <input
                        className={styles.input}
                        placeholder="your-token"
                        spellCheck={false}
                        {...register("auth.token")}
                    />
                </div>
            )}

            {authType === "basic" && (
                <>
                    <div className={styles.field}>
                        <label className={styles.label}>Username</label>
                        <input
                            className={styles.input}
                            placeholder="username"
                            spellCheck={false}
                            {...register("auth.username")}
                        />
                    </div>
                    <div className={styles.field}>
                        <label className={styles.label}>Password</label>
                        <input
                            className={styles.input}
                            type="password"
                            placeholder="password"
                            {...register("auth.password")}
                        />
                    </div>
                </>
            )}
        </div>
    );
}
