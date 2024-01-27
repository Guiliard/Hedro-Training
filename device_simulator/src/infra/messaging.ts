import { Logger } from 'pino'
import { ISend_Messaging, Device_Data } from '../services/interfaces'
import { MqttClient, connect } from 'mqtt'
import { error } from 'console'

export class Messaging implements ISend_Messaging {

    private conection : MqttClient
    private readonly MQTT_HOST : string
    private readonly MQTT_PROTOCOL : string
    private readonly MQTT_USER : string
    private readonly MQTT_PASSWORD : string

    constructor (private readonly logger: Logger) {

        const mqttHost = process.env.MQTT_HOST
        const mqttProtocol = process.env.MQTT_PROTOCOL
        const mqttUser = process.env.MQTT_USER
        const mqttPassWord = process.env.MQTT_PASSWORD

        if (mqttHost === undefined || mqttHost === null || mqttProtocol === undefined || mqttProtocol === null || mqttUser === undefined || mqttUser === null || 
            mqttPassWord === undefined || mqttPassWord === null) { throw new Error('invalid mqtt credentials')}

        this.MQTT_HOST = process.env.MQTT_HOST
        this.MQTT_PROTOCOL = process.env.MQTT_PROTOCOL
        this.MQTT_USER = process.env.MQTT_USER
        this.MQTT_PASSWORD = process.env.MQTT_PASSWORD
    }

    pub(data : Device_Data) : void {

        this.logger.info(`Publishing: ${data.device} ${data.type} ${data.value}`)
        this.conection.publish(`HedroTraining2024/${data.device}/${data.type}`, JSON.stringify(data))
        this.logger.info(`Published`)

    }

    connect() :void {
        
        try{
            this.conection = connect(`${this.MQTT_PROTOCOL}://${this.MQTT_HOST}`, { clientId: this.MQTT_USER, username: this.MQTT_USER, password: this.MQTT_PASSWORD})
            
        } catch (err){
            this.logger.error({ msg: 'something went wrong', error: err })
            throw err
        }
    }
}

