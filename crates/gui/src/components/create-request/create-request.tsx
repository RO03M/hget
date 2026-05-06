import { useForm } from "react-hook-form";
import { Dialog } from "../dialog";
import { TextField } from "../textfield/textfield";
import styles from "./create-request.module.css";
import { Button } from "../button";
import { invoke } from "@tauri-apps/api/core";

interface Props {
    path: string;
    open: boolean;
    onClose: () => void;
}

export function CreateRequestModal(props: Props) {
    const { handleSubmit, register } = useForm();

    const onSubmit = async (data: any) => {
        console.log(props);

        await invoke("create_empty_file", {
            parentPath: props.path,
            requestName: data.request_name
        });
    }

    return (
        <Dialog
            open={props.open}
            onClose={props.onClose}
        >
            <form
                className={styles["create-req-form"]}
                onSubmit={handleSubmit(onSubmit)}
            >
                <TextField
                    {...register("request_name")}
                />
                <Button type="submit">Criar</Button>
            </form>
        </Dialog>
    );
}