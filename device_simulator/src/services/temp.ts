import { IRandom_Generator, ISend_Messaging, Device_Data } from './interfaces'

export class Data_Temp_Generator { 

    constructor(private readonly random : IRandom_Generator, private readonly messaging : ISend_Messaging) {} 

    public Generator() {

        const time = Number(process.env.TIME_TEMP)

        setInterval(() => {

            const rand = this.random.generate()

            const Data : Device_Data = {

                device: "Random",
                type: "TEMP",
                value: String(rand.toFixed(2))
            }

            this.messaging.pub(Data)

            }, time)
    }
}

