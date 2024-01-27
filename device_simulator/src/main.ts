import { LoggerInitializer } from './infra/logger'
import { Messaging } from './infra/messaging'
import { Temp_Random_Generator } from './infra/temp_random'
import { Humidity_Random_Generator } from './infra/humidity_random'
import { Data_Humidity_Generator } from './services/humidity'
import { Data_Temp_Generator } from './services/temp'
import dotenv from 'dotenv'

function main () : void {

    dotenv.config()
    const Log = new LoggerInitializer().init()
    const temp = new Temp_Random_Generator(Log)
    const humidity = new Humidity_Random_Generator(Log)
    const messaging = new Messaging(Log)

    messaging.connect()

    const data_temp_generator = new Data_Temp_Generator(temp, messaging)
    const data_humidity_generator = new Data_Humidity_Generator(humidity, messaging)

    data_temp_generator.Generator()
    data_humidity_generator.Generator()
}

main()