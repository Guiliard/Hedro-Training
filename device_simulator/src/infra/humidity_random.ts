import { Logger } from 'pino'
import { IRandom_Generator } from '../services/interfaces'

const HUMIDITY_MIN = 0.3;
const HUMIDITY_MAX = 0.8;

export class Humidity_Random_Generator implements IRandom_Generator {

    constructor (private readonly logger: Logger){}

    generate() : number {
        
        this.logger.info('Generated Random Data')
        return Math.random() * (HUMIDITY_MAX - HUMIDITY_MIN) + HUMIDITY_MIN
    }

}