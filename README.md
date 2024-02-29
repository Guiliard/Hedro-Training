<h1 align="center">Hedro Training</h1>
<div style="display: inline-block;">
<img align="center" height="20px" width="90px" src=https://img.shields.io/badge/node.js-6DA55F?style=for-the-badge&logo=node.js&logoColor=white>
<img align="center" height="20px" width="90px" src=https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white>
<img align="center" height="20px" width="90px" src=https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white>
<img align="center" height="20px" width="90px" src=https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white>
<img align="center" height="20px" width="90px" src=https://img.shields.io/badge/AWS-%23FF9900.svg?style=for-the-badge&logo=amazon-aws&logoColor=white>
<img align="center" height="20px" width="90px" src=https://img.shields.io/badge/Rabbitmq-FF6600?style=for-the-badge&logo=rabbitmq&logoColor=white>
<img align="center" height="20px" width="90px" src="https://img.shields.io/badge/Made%20for-VSCode-1f425f.svg"/> 
<img align="center" height="20px" width="90px" src="https://img.shields.io/badge/Contributions-welcome-brightgreen.svg?style=flat"/>
</div>
<br>

## Resumo
Este repositório contém o código que visa criar uma simulação de comunicação de dados entre um sensor, um broker MQTT, o RabbitMQ e a AWS. A ideia central é reproduzir um ambiente controlado que imita a troca de informações em um sistema distribuído. O projeto foi conduzido sobre a orientação do Engenheiro de Software <a href="https://github.com/ralvescosta" target="_blank">Rafael Costa</a> (Hedro - S.I.), sendo caracterizado como o produto central do treinamento de estágio Hedro 2024, envolvendo tecnologias como: Node.js, TypeScript, Rust, DockerFile, AWS e VSCode. 

## Instruções de Execução

No terminal, execute as seguintes instruções:

### Clonar o repositório:

```
git clone https://github.com/Guiliard/Hedro-Training.git
cd Hedro-Training
```

### Construir o contêiner do simulador de dispositivo (device_simulator):

```
cd device_simulator/
docker build . -t device_simulator
cd ..
```

### Construir o contêiner da ponte entre o broker MQTT (EMQX) e o broker RabbitMQ (rmq-bridge):

```
cd rmq-bridge/
docker build . -t rmq-bridge
cd ..
```

### Construir o contêiner do consumidor de mensagens do broker RabbitMQ e ponte para a AWS (rmq-consumer):

```
cd rmq-bridge/
docker build . -t rmq-consumer
cd ..
```

### Executar a aplicação com a utilização do docker-compose:

```
sudo docker-compose up -d
sudo docker-compose ps
sudo docker-compose down
```

## Especificações do Dispositivo Utilizado

| Componentes            | Detalhes                                                                                         |
| -----------------------| -----------------------------------------------------------------------------------------------  |
|  `Processador`         | Intel(R) Core(TM) i7-1065G7 CPU @ 1.30GHz   1.50 GHz                                             |
|  `RAM Instalada`       | 12.0 GB (Utilizável: 11.8 GB)                                                                    |
|  `Tipo de Sistema`     | Sistema Operacional de 64 bits, processador baseado em x64                                       |
|  `Sistema Operacional` | Linux Pop!_OS 22.04 LTS                                                                           |

## Referências

[1] ECLIPSE - Repositório GitHub, @eclipse: paho.mqtt.rust - Disponível em: https://github.com/eclipse/paho.mqtt.rust. Acessado em 22 de Janeiro de 2024.

[2] RABBITMQ - Repositório GitHub, @rabbitmq: rabbitmq-tutorials - Disponível em: https://github.com/rabbitmq/rabbitmq-tutorials. Acessado em 27 de Janeiro de 2024.

[3] AWS - AWS Documentation. Disponível em: <https://docs.aws.amazon.com/>. Acessado em: 3 de Fevereiro de 2024.

[4] Docker - Get Docker. Disponível em: <https://docs.docker.com/get-docker/>. Acessado em: 17 de Janeiro de 2024.
