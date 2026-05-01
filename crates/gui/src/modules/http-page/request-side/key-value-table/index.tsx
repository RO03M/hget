import { useFieldArray, useFormContext } from "react-hook-form";
import { RequestFormHttpRequest, emptyRow } from "../types";
import styles from "./key-value-table.module.css";

interface Props {
    name: "params" | "headers";
}

export function KeyValueTable({ name }: Props) {
    const { register, control } = useFormContext<RequestFormHttpRequest>();
    const { fields, append, remove } = useFieldArray({ control, name });

    return (
        <div className={styles.container}>
            <div className={styles.table}>
                <div className={styles.header}>
                    <span className={styles.colCheck} />
                    <span className={styles.colName}>Name</span>
                    <span className={styles.colValue}>Value</span>
                    <span className={styles.colAction} />
                </div>

                {fields.map((field, index) => (
                    <div key={`${field.name}-${index}-${field.value}`} className={styles.row}>
                        <span className={styles.colCheck}>
                            <input
                                type="checkbox"
                                {...register(`${name}.${index}.enabled`)}
                            />
                        </span>
                        <span className={styles.colName}>
                            <input
                                className={styles.cellInput}
                                placeholder="name"
                                spellCheck={false}
                                {...register(`${name}.${index}.name`)}
                            />
                        </span>
                        <span className={styles.colValue}>
                            <input
                                className={styles.cellInput}
                                placeholder="value"
                                spellCheck={false}
                                {...register(`${name}.${index}.value`)}
                            />
                        </span>
                        <span className={styles.colAction}>
                            <button
                                type="button"
                                className={styles.deleteBtn}
                                onClick={() => remove(index)}
                                aria-label="Remove row"
                            >
                                ✕
                            </button>
                        </span>
                    </div>
                ))}
            </div>

            <button type="button" className={styles.addRow} onClick={() => append(emptyRow())}>
                + Add row
            </button>
        </div>
    );
}
