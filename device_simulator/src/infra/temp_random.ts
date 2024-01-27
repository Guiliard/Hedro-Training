import { Logger } from 'pino'
import { IRandom_Generator } from '../services/interfaces'

const TEMP_MIN = 10;
const TEMP_MAX = 45;

export class Temp_Random_Generator implements IRandom_Generator {

    constructor (private readonly logger: Logger) {}

    generate() : number {
        
        this.logger.info('Generated Random Data')
        return Math.random() * (TEMP_MAX - TEMP_MIN) + TEMP_MIN
    }

}