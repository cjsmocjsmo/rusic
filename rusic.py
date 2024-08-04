import argparse
import os
import subprocess

CWD = os.getcwd()

def docker_command(arch, version):
    docker_command = [
            "docker", "run",
            "--name", f"rusic{arch}",
            "-e", "RUS_DB_PATH=/usr/share/rusic/rusic/db/rusic.db",
            "-e", "RUS_DB_CHECK_FILE_PATH=/usr/share/rusic/rusic/db/db_check_file.txt",
            "-e", "RUS_THUMBS=/usr/share/rusic/rusic/thumbnails",
            "-e", "RUS_NFOS=/usr/share/rusic/rusic/nfo",
            "-e", "RUS_RAW_HTTP=192.168.0.74",
            "-e", "RUS_HTTP_ADDR=http://192.168.0.74",
            "-e", "RUS_PORT=:8080",
            "-d",
            "-p", "8080:8080",
            "-v", "/usr/share/rusicsetup/rusicsetup/db/:/usr/share/rusic/rusic/db/",
            "-v", "/usr/share/rusicsetup/rusicsetup/thumbnails/:/usr/share/rusic/rusic/thumbnails/",
            "-v", f"{os.path.expanduser('~')}/Music:/usr/share/rusic/rusic/Music",
            f"rusic{arch}:{version}"
        ]
    return docker_command

def rusic_install(version, docker_file, arch):
    if not os.path.exists("/usr/share/rusicsetup/rusicsetup"):
        print("Rusic Setup not found. Please install it first.")
        exit(1)

    if os.path.exists(docker_file):
        print(f"Installing Rusic{arch}:{version}")
        subprocess.run(["docker", "build", "-t", f"rusic{arch}:{version}", "."])
        dcommand = docker_command(arch, version)
        subprocess.run(dcommand)
        subprocess.run(["rm", "-f", f"{CWD}/Dockerfile"])

def rusic_update(version, docker_file, arch):
    print(f"Updating Rusic{arch}:{version}")
    subprocess.run(["rm", "-f", f"{CWD}/Dockerfile"])
    subprocess.run(["docker", "stop", f"rusic{arch}"])
    print("Docker container stopped.")
    subprocess.run(["docker", "rm", f"rusic{arch}"])
    print("Docker container removed.")
    subprocess.run(["git", "pull"])
    print("Git repository updated.")
    subprocess.run(["cp", "-pvr", docker_file, CWD])
    print("New Dockerfile copied.")
    subprocess.run(["docker", "build", "-t", f"rusic{arch}:{version}", "."])
    print("Docker image built.")
    dcommand = docker_command(arch, version)
    subprocess.run(dcommand)
    print("Docker container started.")
    subprocess.run(["rm", "-f", f"{CWD}/Dockerfile"])

def rusic_delete(version, arch):
    print(f"Deleting Rusic{arch}:{version}")
    subprocess.run(["docker", "stop", f"rusic{arch}"])
    print("Docker container stopped.")
    subprocess.run(["docker", "rm", f"rusic{arch}"])
    print("Docker container removed.")
    subprocess.run(["docker", "rmi", f"rusic{arch}:{version}"])
    print("Docker image removed.")
    print("The Rusic folder can now be removed")


def main():
    parser = argparse.ArgumentParser(description="CLI for Rusic music server.")
    parser.add_argument("version", type=str, help="Version of the software")
    parser.add_argument("-i", "--install", action="store_true", help="Install the program")
    parser.add_argument("-u", "--update", action="store_true", help="Update the program")
    parser.add_argument("-d", "--delete", action="store_true", help="Delete the program")

    args = parser.parse_args()

    
    docker_32_file = os.path.join(CWD, "RPI", "32", "Dockerfile")
    docker_64_file = os.path.join(CWD, "RPI", "64", "Dockerfile")
    print(docker_32_file)
    print(docker_64_file)

    # count1 = args.version.replace(".", "")
    # count = int(count1) + 1 - 1
    # minusone = count - 1

    if os.uname().machine == "armv7l":
        subprocess.run(["cp", "-pvr", docker_32_file, CWD])
        arch = "32"
        if args.install:
            print(f"Installing Rusic:{args.version}")
            rusic_install(args.version, docker_32_file, arch)
        elif args.update:
            print(f"Updating Rusic:{args.version}")
            rusic_update(args.version, docker_32_file, arch)
        elif args.delete:
            print(f"Deleting Rusic:{args.version}")
            rusic_delete(args.version, arch)
        else:
            print("No action specified. Use -i, -u, or -d.")
    elif os.uname().machine == "aarch64":
        subprocess.run(["cp", "-pvr", docker_64_file, CWD])
        arch = "64"
        if args.install:
            print(f"Installing Rusic:{args.version}")
            rusic_install(args.version, docker_64_file, arch)
        elif args.update:
            print(f"Updating Rusic:{args.version}")
            rusic_update(args.version, docker_64_file, arch)
        elif args.delete:
            print(f"Deleting Rusic:{args.version}")
            rusic_delete(args.version, docker_64_file, arch)
        else:
            print("No action specified. Use -i, -u, or -d.")
    else:
        print("Invalid architecture. Use 32 or 64.")

if __name__ == "__main__":
    main()