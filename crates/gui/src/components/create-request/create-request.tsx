import { useForm } from "react-hook-form";
import { Dialog } from "../dialog";
import { TextField } from "../textfield/textfield";
import styles from "./create-request.module.css";
import { Button } from "../button";
import { NewRequestHeader } from "./header";
import { saveRequest } from "../../requests/save_request";
import { join } from "@tauri-apps/api/path";

interface CreateRequestForm {
    request_name: string;
    url: string;
}

interface Props {
    path: string;
    open: boolean;
    onClose: () => void;
}

export function CreateRequestModal(props: Props) {
    const { handleSubmit, register } = useForm<CreateRequestForm>();

    const onSubmit = async (data: CreateRequestForm) => {
        const path = await join(props.path, `${data.request_name}.http`);

        console.log("Creating new request at", path, " with ", data);

        await saveRequest({
            body: null,
            headers: [],
            method: "GET",
            name: data.request_name,
            url: data.url,
        }, path);
    }

    return (
        <Dialog
            open={props.open}
            onClose={props.onClose}
        >
            <NewRequestHeader onClose={props.onClose} />
            <form
                className={styles["create-req-form"]}
                onSubmit={handleSubmit(onSubmit)}
            >
                <label>Request Name</label>
                <TextField
                    placeholder={"Request Name"}
                    {...register("request_name")}
                />
                <br/>
                <label>URL</label>
                <TextField
                    placeholder={"URL"}
                    {...register("url")}
                />

                <div
                    style={{
                        marginTop: 8,
                        display: "flex",
                        justifyContent: "flex-end",
                        gap: 8,
                    }}
                >
                    <Button
                        variant="text"
                        type="button"
                        onClick={props.onClose}
                    >
                        Cancelar
                    </Button>
                    <Button type="submit">Criar</Button>
                </div>
            </form>
        </Dialog>
    );
}