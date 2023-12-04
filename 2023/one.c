#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/_types/_ssize_t.h>
#include <sys/mman.h>

void mutate_line_cardinals(char *line);
void part_one();

const char n_set[][10] = {"zero", "one", "two",   "three", "four",
                          "five", "six", "seven", "eight", "nine"};

int main(int argc, char *argv[]) {
  if (!argv[1]) {
    printf("missing argument, try:\n");
    printf("1     run part 1 \n");
    printf("2     run part 2 \n");
    exit(EXIT_FAILURE);
  }
  FILE *file;
  char line[256];

  file = fopen("./one", "r");

  if (!file) {
    printf("file not found");
    exit(EXIT_FAILURE);
  }

  // loop lines

  char c;
  char first = '\0', second = '\0';
  size_t acc = 0;
  while (fgets(line, sizeof(line), file)) {
    // part 2
    //
    // here lies the problem..
    //  // eighthree shoud be 83...
    //  // this implementation considers it as 8hree
    // bad approach, does not consider overlap
    // TODO: consider overlap
    if (*argv[1] == '2') {
      mutate_line_cardinals(line);
    }

    for (size_t i = 0; i < strlen(line); i++) {
      c = line[i];

      if (c >= '0' && c <= '9') {
        if (!first) {
          first = c;
        }
        second = c;
      }
    }

    int dig1 = first - '0';
    int dig2 = second - '0';

    acc = acc + dig1 * 10 + dig2;
    /* printf("%c %c   ACC:%lu\n", first, second, acc); */

    first = '\0';
    second = '\0';
  };

  fclose(file);
  printf("%lu \n", acc);

  return EXIT_SUCCESS;
}

void mutate_line_cardinals(char *line) {

  printf("%s", line);
  while (1) {
    bool found_match = false;
    char *matches[10] = {NULL};
    for (size_t i = 0; i < 10; i++) {
      char *match = strstr(line, n_set[i]);
      if (match) {
        found_match = true;
        matches[i] = match;
      }
    }
    // evaluate all matches; find first and last
    if (found_match) {
      char *first_ptr = &line[strlen(line)];
      char *last_ptr = &line[0];
      size_t first_digit;
      size_t last_digit;

      for (size_t i = 0; i < 10; i++) {
        if (matches[i] == NULL) {
          continue;
        }

        // first
        if (matches[i] <= first_ptr) {
          first_ptr = matches[i];
          first_digit = i;
        }
        // last
        if (matches[i] + strlen(n_set[i]) >= last_ptr) {
          last_ptr = matches[i];
          last_digit = i;
        }
      }
      // replace characters in line with first
      size_t digit_len = strlen(n_set[first_digit]);
      for (size_t j = 0; j < digit_len; j++) {
        char digit = first_digit + '0';
        *(first_ptr + j) = digit;
      }
      // replace characters in line with first
      digit_len = strlen(n_set[last_digit]);
      for (size_t j = 0; j < digit_len; j++) {
        char digit = last_digit + '0';
        *(last_ptr + j) = digit;
      }
    } else {
      break;
    }
  }
  printf("%s\n", line);
}
