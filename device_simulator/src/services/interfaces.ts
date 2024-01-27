type Device_Data = {
    
    device: string,
    value: string,
    type: "TEMP" | "HUMIDITY"
}

interface IRandom_Generator {

    generate() : number
}

interface ISend_Messaging {

    pub(data : Device_Data) : void
}

export { IRandom_Generator, ISend_Messaging, Device_Data }