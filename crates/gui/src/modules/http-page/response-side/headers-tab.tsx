interface Props {
    headers: [string, string][];
}

export function HeadersTab(props: Props) {
    return (
        <table
            style={{
                margin: 10,
                width: "100%",
                tableLayout: "fixed",
                wordBreak: "break-all"
            }}
        >
            <thead>
                <th>Name</th>
                <th>Value</th>
            </thead>
            <tbody>
                {props.headers.map((header) => (
                    <tr>
                        <td>{header[0]}</td>
                        <td>{header[1]}</td>
                    </tr>
                ))}
            </tbody>
        </table>
    );
}