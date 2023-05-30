# OCI runtime

## Authors
Déodat Père & Vianney Ruhlmann

## Idée du projet

Ce projet est un runtime pour conteneur en Rust, remplissant des fonctions similaires a ```runc```. Nous nous sommes inspiré de la norme OCI (Open Container Initiative), décrivant le cahier des charges d'un runtime pour conteneur avec lequel on pourrait faire tourner Docker ou Kubernetes à la place de runc. Etant donné la complexité du projet, il n'était pas réaliste d'implémenter tout le cahier des charges de la norme OCI, et nous nous sommes donc rabattus sur un cahier des charges plus limité.

## Tutoriel

Pour utiliser le runtime vous avez besoin d'un dossier qui servira de filesystem pour le conteneur.
Une manière simple de le générer est d'utiliser docker.

``` shell
docker export $(docker create alpine) --output="alpine.tar"
tar -xf alpine.tar -C alpine
```

Il faut ensuite rédiger une configuration, voici un exemple:

``` json
{
    "root": {
        "path": "chemin_vers_le_filesystem"
    },
    "process": {
        "cwd": "/",
        "args": [
            "/bin/echo",
            "hello"
        ],
        "user": {
            "uid": 1,
            "gid": 1
        }
    },
    "linux": {
        "namespaces": [
            {"type": "pid"},
            {"type": "network"},
            {"type": "mount"},
            {"type": "ipc"},
            {"type": "uts"},
            {"type": "user"},
            {"type": "cgroup"}
        ]
    }
}
```

Vous pouvez maintenant créer votre conteneur et le démarrer

``` shell
cargo run create id config.json
cargo run start id
```



## Fonctionnalités

### Create

Avec la commande ```oci-runtime create <ID> <Path>``` . 
Le dossier ```Path``` contient une fichier de statut décrivant le conteneur a faire tourner, notamment le processus a executer. On crée pour ce conteneur un fichier de status, dans lequel on , effectues les opérations d'isolation dans ```Path```, puis se place en attente de la commande ```start```. L'identifiant ```ID``` doit être unique. Place l'état du conteneur à ```Created```.

### State

Avec la commande ```oci-runtime state <ID>```.
Affiche les informations de statut suivante du conteneur:
- Etat, ```Created``` ou ```Running```
- Bundle, chemin vers le dossier image
- PID, PID du processus dans le namespace de l'hôte
- ID, identifiant unique du conteneur

### Start

Avec la commande ```oci-runtime start <ID>```.
Envoie via des sockets UNIX le signal de démarrer le programme spécifier au processus en attente dans le conteneur. Change l'état du conteneur vers ```Running```.

### Kill

Avec la commande ```oci-runtime kill <ID> <SIG>```.
Envoie le signal ```SIG``` au processus du conteneur d'identifiant ```ID```.
Les processus sont passés sous la forme décrite ici[https://tikv.github.io/doc/nix/sys/signal/enum.Signal.html].

### Delete

Avec la commande ```oci-runtime delete <ID>```.
Supprime le dossier du conteneur d'identifiant ```ID```, ainsi que le fichier de statut.
