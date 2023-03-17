export class Snowflake {
    constructor(dataCenterId: number, workerId: number);

    nextId(): string
}