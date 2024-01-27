import pino, { Logger } from 'pino'

export class LoggerInitializer {

    public init() : Logger {

        return pino({ level: process.env.LOG_LEVEL || 'debug' })
    }
}