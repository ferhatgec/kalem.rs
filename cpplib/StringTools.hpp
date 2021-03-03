// MIT License
//
// Copyright (c) 2020 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//

// Modified for Kalem's !include_dir & import functions testing.

#ifndef STRING_TOOLS_HPP
#define STRING_TOOLS_HPP

#include <iostream>
#include <string>

namespace stringtools {
	static std::string EraseAllSubString(std::string & mainString, const std::string & erase) {
        size_t pos = std::string::npos;

        while((pos = mainString.find(erase)) != std::string::npos) {
            mainString.erase(pos, erase.length());
  		}

  		return mainString;
  	}

	/* return as string */
	static std::string GetBetweenString(std::string oStr, std::string sStr1, std::string sStr2) {
	    int start = oStr.find(sStr1);

	    if (start >= 0) {
	        std::string tstr = oStr.substr(start + sStr1.length());

	        int stop = tstr.find(sStr2);

	        if (stop > 1)
	            return oStr.substr(start + sStr1.length(), stop);
	    }

	    return "error";
	}
}

#endif // STRING_TOOLS_HPP
