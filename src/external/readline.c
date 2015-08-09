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
	//Catch Carriage Returns and new lines
	if(output == '\r' || output == '\n') {
		return -5;
	}
	else if(output == 127) { //Backspace
		return -6;
	} else if(output == '\t') {
		return -10;
	}
	//Handling Arrow Keys
	if(output == '\033') { //if first value is escape
		getch(); //skips the [
		switch(getch()) {
			case 'A': //Up
				return -1;
			case 'B': //Down
				return -2;
			case 'C': //Right
				return -3;
			case 'D': //Left
				return -4;
		}
	}

	return output;
}

//Cursor Related Functions
void backspace(int back) {
	if(back == 0) {
		fputs("\033[D\033[K",stdout);
	} else if(back == 1) {
		fputs("\033[D ",stdout);
	}
}

void left(int boolean) {
	if(boolean == 1)
		fputs("\033[D",stdout);
}

void right(int boolean) {
	if(boolean == 1)
		fputs("\033[C",stdout);
}

void go_back(char n[], int length) {
	int i = 0;
	int j = 0;
	for(i; i < length; i++) {
		printf("%c",n[i]);
	}
	for(j; j < length; j++) {
		fputs("\033[D",stdout);
	}
}

void clear_to_end(void){
	fputs("\033[K",stdout);
}
