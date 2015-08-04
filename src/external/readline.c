#include "stdio.h"
#include "termios.h"
static struct termios old, new;
void initTermios(int echo) {
	tcgetattr(0, &old);
	new = old;
	new.c_lflag &= ~ICANON;
	new.c_lflag &= echo ? ECHO : ~ECHO;
	tcsetattr(0, TCSANOW, &new);
}

void resetTermios(void) {
	tcsetattr(0,TCSANOW,&old);
}

char getch_(int echo) {
	char ch;
	initTermios(echo);
	ch = getchar();
	resetTermios();
	return ch;
}

char getch(void) {
	return getch_(0);
}

int get_input() {
	char output = getch();

	//Catch Carriage Returns.
	if(output == 10)
		return -5;

	//Handling Arrow Keys
	if(output == '\033') { //if first value is escape
		getch(); //skips the [
		switch(getch()) {
			case 'A':
				return -1;
			case 'B':
				return -2;
			case 'C':
				return -3;
			case 'D':
				return -4;
		}
	}

	//Count it as the end of a line
	if(output == '\n')
		return -5;

	return output;
}
