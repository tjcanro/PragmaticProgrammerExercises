#include <stdio.h>
#include <stdbool.h>

int SelectPen(int dir, int amount) {
    printf("Pen selected\n");
    return 0;  // Placeholder return
}

int PenDown(int dir, int amount) {
    printf("Pen down\n");

    return 0;  // Placeholder return
}

int PenUp(int dir, int amount) {
    printf("Pen up\n");

    return 0;  // Placeholder return
}

int PenMovement(int dir, int amount) {
    printf("Pen move to %c, %d\n", dir, amount);
    return 0;  // Placeholder return
}

typedef struct {
    char command;
    bool hasArgs;
    int (*function)(int, int); 
} Command_t;

static Command_t commands[] = {
    {'P', true, SelectPen},
    {'D', false, PenDown},
    {'U', false, PenUp},
    {'W', true, PenMovement},
    {'N', true, PenMovement},
    {'E', true, PenMovement},
    {'S', true, PenMovement},
};

int getArg(const char *buf, int *result){
    return sscanf(buf, "%d", result) == 1;
}

Command_t *findCommand(char command) {
    for (int i = 0; i < sizeof(commands) / sizeof(commands[0]); i++) {
        if (commands[i].command == command) {
            return commands + i;
        }
    }
    fprintf(stderr, "Unknown command %c \n", command);
    return NULL;
}

int main() {
    char buff[256];

    while (fgets(buff, sizeof(buff), stdin)) {
        Command_t *command = findCommand(*buff); 

        if (command) {
            int arg = 0;

            if (command->hasArgs && !getArg(buff + 1, &arg)) {
                printf("Invalid argument\n");
                continue;
            }

            command->function(*buff, arg);
        } else {
            printf("Invalid command\n");
        }
    }
    return 0;
}
