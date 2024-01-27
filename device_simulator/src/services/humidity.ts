import { IRandom_Generator, ISend_Messaging, Device_Data } from './interfaces'

export class Data_Humidity_Generator { 

    constructor(private readonly random : IRandom_Generator, private readonly messaging : ISend_Messaging) {} 

    public Generator() {

        const time = Number(process.env.TIME_HUMIDITY)

        setInterval(() => { 

            const rand = this.random.generate()

            const Data : Device_Data = {

                device: "Random",
                type: "HUMIDITY",
                value: String(rand.toFixed(2))
            }

            this.messaging.pub(Data)
            
        }, time)
    }

}