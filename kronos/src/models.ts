import {DataTypes, Model, type Infer} from "d1-orm";

const Nodes = new Model(
    {
        tableName: "nodes",
        primaryKeys: "id",
        uniqueKeys: [["serial"]]
    },
    {
        id: {
            type: DataTypes.STRING,
            notNull: true
        },
        serial: {
            type: DataTypes.STRING,
            notNull: true
        }
    }
)
type Node = Infer<typeof Nodes>;

export { Nodes };